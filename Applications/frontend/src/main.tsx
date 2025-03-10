import React from 'react';
import ReactDOM from 'react-dom/client';
import { createBrowserRouter, RouterProvider } from 'react-router-dom';
import PhotoBooth from './GoogleBooth';
import { GoogleOAuthProvider } from '@react-oauth/google';
import './index.css';
// import Wallet from './pages/Wallet';
import TransferAssetPage from './pages/transfer-asset';
import swap from './pages/swap';
const router = createBrowserRouter([
  { path: '/', Component: PhotoBooth },
  // { path: '/contact/:wallet', Component: Wallet },
  { path: '/transfer-assets', Component: TransferAssetPage },
  { path: '/swap', Component: swap },
]);
const CLIENT_ID = '';
  <React.StrictMode>
    <GoogleOAuthProvider clientId={CLIENT_ID}>
      <RouterProvider router={router} />
    </GoogleOAuthProvider>
  </React.StrictMode>,
);
