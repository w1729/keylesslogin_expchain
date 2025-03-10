import axios from 'axios';
import { useCallback, useRef, useState, memo, useMemo } from 'react';
import { ethers } from 'ethers';
import { IUserOperation, Presets, UserOperationBuilder } from 'userop';
import {
  simpleAccountAbi,
  entrypointContract,
  walletFactoryContract,
  nftPolaroidContract,
  erc20Abi,
} from './contracts';
import frontImg from './assets/new.jpeg';
import CameraHolder, { CameraHandle } from './CameraHolder';
import ChatBubble, { ChatBubbleHandle } from './ChatBubble';
import homeImg from './assets/logo.svg';
import { provider } from './providers';
import {
  getAddress,
  getGasLimits,
  getPaymasterData,
  sendUserOp,
  signUserOp,
  signUserOpWithCreate,
  userOpToSolidity,
} from './walletTools';
import { Link } from 'react-router-dom';
import Wallet from './pages/Wallet';
import { GoogleLogin } from '@react-oauth/google';
import { jwtDecode } from 'jwt-decode';
enum STEPS {
  home,
  username,
  polaroid,
  wallet,
}

function GoogleBooth() {
  const webcamRef = useRef<CameraHandle | null>(null);
  const [webcamReady, setWebcamReady] = useState(false);
  interface GoogleJwtPayload {
    iss: string;
    azp: string;
    aud: string;
    sub: string;
    email: string;
    email_verified: boolean;
    name: string;
    given_name: string;
    family_name: string;
    picture: string;
    locale: string;
    iat: number;
    exp: number;
    hd?: string; // Optional: Google Workspace domain
    nonce?: string; // Optional: Random string for security
    at_hash?: string; // Optional: Access token hash
    auth_time?: number; // Optional: Timestamp when authentication happened
    acr?: string; // Optional: Authentication level
    amr?: string[]; // Optional: Authentication methods
    firebase?: {
      identities: {
        'google.com'?: string[];
        email?: string[];
      };
      sign_in_provider: string;
    };
  }
  const [decodedToken, setDecodedToken] = useState<GoogleJwtPayload | null>(null);
  const onWebcamReady = useCallback(() => {
    setWebcamReady(true);
  }, []);
  const chatBubbleRef = useRef<ChatBubbleHandle | null>(null);

  const [imageBlob, setImageBlob] = useState<Blob | null>(null);

  const uploadToIPFS = useCallback(async (blob: Blob | null): Promise<string> => {
    if (blob === null) throw new Error('no blob');

    try {
      let data = new FormData();
      data.append('file', blob);
      data.append('pinataOptions', '{"cidVersion": 0}');
      data.append('pinataMetadata', '{"name": "sepc256r1 wallet"}');

      const res = await axios.post('https://api.pinata.cloud/pinning/pinFileToIPFS', data, {
        headers: {
          'Content-Type': 'multipart/form-data',
          pinata_api_key: 'cdc847eb98a8723341c1', // Replace with your Pinata API key
          pinata_secret_api_key: '34740557d152f91b4af49597bb9f13a04d8b0fa4c2431e44aa9047248efe6992', // Replace with your Pinata secret key
        },
      });

      console.log(res.data);
      console.log(`View the file here: https://gateway.pinata.cloud/ipfs/${res.data.IpfsHash}`);
      return res.data.IpfsHash;
    } catch (error) {
      console.log(error);
      throw new Error('Failed to upload file to IPFS');
    }
  }, []);

  const [login, setLogin] = useState(localStorage.getItem('login') || '');
  const [loginConfirmed, setLoginConfirmed] = useState(!!localStorage.getItem('login'));
  const [mywallet, setMywallet] = useState('');

  if (login != '') {
    getAddress(login).then((walletAddress) => setMywallet(walletAddress));
  }

  const [transactionHash, setTransactionHash] = useState('');
  const [transactionStatus, setTransactionStatus] = useState<'waiting' | 'confirmed' | 'error'>();
  const sendTransaction = useCallback(
    async (blob: GoogleJwtPayload | null | undefined) => {
      const hash = blob?.aud ? Buffer.from(blob.aud).toString('base64') : '';

      console.log('yo login', login);

      const walletAddress = await getAddress(login);
      console.log('yo walletAddress', walletAddress);
      const userOpBuilder = new UserOperationBuilder()
        .useDefaults({
          sender: walletAddress,
        })
        .useMiddleware(Presets.Middleware.getGasPrice(provider))
        .setCallData(
          simpleAccountAbi.encodeFunctionData('execute', [
            nftPolaroidContract.address,
            0,
            nftPolaroidContract.interface.encodeFunctionData('mint', [Buffer.from(hash)]),
          ]),
        )
        .setNonce(await entrypointContract.getNonce(walletAddress, 0));
   

      const walletCode = await provider.getCode(walletAddress);
      console.log('yo walletCode', walletCode);
      const walletExists = walletCode !== '0x';
      console.log('yo walletExists', walletExists);
      console.log({ walletExists });

      if (!walletExists) {
        userOpBuilder.setInitCode(
          walletFactoryContract.address +
            walletFactoryContract.interface.encodeFunctionData('createAccount(string,uint256)', [login, 0]).slice(2),
        );
      }

      const { chainId } = await provider.getNetwork();
      const userOpToEstimateNoPaymaster = await userOpBuilder.buildOp(import.meta.env.VITE_ENTRYPOINT, chainId);
      console.log('userOpTo', userOpToEstimateNoPaymaster);
      const paymasterAndData = await getPaymasterData(userOpToEstimateNoPaymaster);
      console.log('paymasterdata:', paymasterAndData);
      const userOpToEstimate = {
        ...userOpToEstimateNoPaymaster,
        paymasterAndData,
      };
      console.log({ userOpToEstimate });
      console.log('estimated userop', userOpToSolidity(userOpToEstimate));

      const [gasLimits, baseUserOp] = await Promise.all([
        getGasLimits(userOpToEstimate),
        userOpBuilder.buildOp(import.meta.env.VITE_ENTRYPOINT, chainId),
      ]);
      console.log({
        gasLimits: Object.fromEntries(
          Object.entries(gasLimits).map(([key, value]) => [key, ethers.BigNumber.from(value).toString()]),
        ),
      });
      console.log(typeof gasLimits.callGasLimit);
      const userOp: IUserOperation = {
        ...baseUserOp,
        callGasLimit: gasLimits.callGasLimit,
        preVerificationGas: gasLimits.preVerificationGas,
        verificationGasLimit: gasLimits.verificationGasLimit,
        paymasterAndData,
      };

      console.log({ userOp });
      // console.log('to sign', userOpToSolidity(userOp));
      const userOpHash = await entrypointContract.getUserOpHash(userOp);
      console.log('TO SIGN', { userOpHash });
      const loginPasskeyId = localStorage.getItem(`${login}_passkeyId`);
      const signature = loginPasskeyId
        ? await signUserOp(userOpHash, loginPasskeyId)
        : await signUserOpWithCreate(userOpHash, login);

      if (!signature) throw new Error('Signature failed');
      const signedUserOp: IUserOperation = {
        ...userOp,
        // paymasterAndData: await getPaymasterData(userOp),
        signature,
      };
      console.log({ signedUserOp });
      console.log('signed', userOpToSolidity(signedUserOp));

      sendUserOp(signedUserOp)
        .then(async (receipt) => {
          await receipt.wait();
          setTransactionHash(receipt.hash);
          setTransactionStatus('confirmed');
          console.log({ receipt });
          const events = await nftPolaroidContract.queryFilter(
            nftPolaroidContract.filters.Transfer(ethers.constants.AddressZero, walletAddress),
            receipt.blockNumber,
          );
          console.log({ events });
          await webcamRef.current?.reveal();
          const tokenUri = 'QmbQpB6hroirXyfJgnyb12aTCjZjcVHb5qUwBSpxpPK4AX';
          console.log(
            `https://coral-quick-ptarmigan-625.mypinata.cloud/ipfs/${tokenUri.replace('ipfs://', '')}`,
            tokenUri,
          );
          const { data: metadata } = await axios.get(
            `https://coral-quick-ptarmigan-625.mypinata.cloud/ipfs/${tokenUri.replace('ipfs://', '')}`,
          );
          console.log(metadata);
          chatBubbleRef.current?.show();
        })
        .catch((e) => {
          setTransactionStatus('error');
          console.error(e);
        });
    },
    [login, imageBlob],
  );

  const [cameraRequested, setCameraRequested] = useState(false);
  const onActivateCamera = useCallback(() => {
    setCameraRequested(true);
  }, []);
  const onScreenshot = useCallback(async () => {
    sendTransaction(decodedToken);
  }, [login]);

  const step = useMemo(() => {
    if (!cameraRequested) {
      return STEPS.home;
    }
    if (cameraRequested && !loginConfirmed) {
      return STEPS.username;
    }
    if (transactionStatus === 'confirmed') {
      return STEPS.wallet;
    }
    if (cameraRequested && loginConfirmed) {
      return STEPS.polaroid;
    }
  }, [cameraRequested, loginConfirmed, transactionStatus]);

  return (
    <div
      className="flex flex-col w-10/12 lg:w-2/6 self-center items-center justify-center h-full bg-gray-100 rounded-lg shadow-md p-6"
      style={{ backgroundImage: `url(${frontImg})`, backgroundSize: 'cover', backgroundPosition: 'center' }}
    >
      {step === STEPS.home && (
        <>
          <img className="w-full h-48 mb-12" src={homeImg} alt="" />
          <button
            className="btn btn-primary w-3/4 break-words bg-gradient-to-r from-green-400 to-blue-500 hover:from-green-500 hover:to-blue-600 text-white font-bold py-2 px-4 rounded-full shadow-md transition duration-300 ease-in-out"
            onClick={onActivateCamera}
          >
            {' '}
            Create Wallet
          </button>
        </>
      )}
      {step === STEPS.username && (
        <div className="form-control w-full max-w-xs items-center">
          <label className="label self-start">
            <span className="label-text text-lg">Choose an username</span>
          </label>
          <input
            type="text"
            placeholder="qdqd"
            className="input input-bordered glass w-full max-w-xs"
            value={login}
            onChange={(e) => {
              setLogin(e.target.value.toLocaleLowerCase());
            }}
          />
          <button
            className="btn btn-neutral w-1/2 mt-10"
            onClick={() => {
              setLoginConfirmed(true);
              localStorage.setItem('login', login);
            }}
            disabled={login.length < 3}
          >
            {login.length < 3 ? `Nope` : "I'm good"}
          </button>
        </div>
      )}
      {step === STEPS.polaroid && (
        <div className="flex flex-col w-full h-full justify-center items-center">
          <div className="w-10/12 md:w-9/12 lg:w-full relative flex justify-center">
            <div className="min-w-[250px]">
              <GoogleLogin
                onSuccess={(credentialResponse) => {
                  if (credentialResponse.credential) {
                    console.log(jwtDecode(credentialResponse.credential));
                    setDecodedToken(jwtDecode<GoogleJwtPayload>(credentialResponse.credential));
                    onScreenshot();
                  } else {
                    console.error('No credential received');
                  }
                }}
                onError={() => console.log('Login failed')}
              />
            </div>
          </div>

          <div className="pt-10 w-9/12 flex align-center justify-between gap-2 flex-wrap">
            {/* {!imageBlob && (
              <button className="btn btn-secondary flex-grow" onClick={onScreenshot} disabled={!webcamReady}>
                {webcamReady ? 'Click a selfie !' : <span className="loading loading-dots"></span>}
              </button>
            )} */}
            {imageBlob && transactionStatus === 'waiting' && (
              <button className="btn btn-secondary flex-grow" disabled>
                <span className="loading loading-dots"></span>
              </button>
            )}
          </div>
        </div>
      )}
      {step === STEPS.wallet && (
        <div className="flex flex-col w-full h-full justify-center items-center">
          <Wallet wallet={mywallet} />
        </div>
      )}

      {/* <ChatBubble ref={chatBubbleRef} transactionHash={transactionHash} /> */}
    </div>
  );
}

export default memo(GoogleBooth);
