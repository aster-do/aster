import React from 'react';
import ReactDOM from 'react-dom/client';
import './index.css';
import { RouterProvider, createBrowserRouter } from 'react-router-dom';
import reportWebVitals from './reportWebVitals';
import AppLayout from './common/components/AppLayout';
import getRoutes, { navigationRoutes } from './common/navigation';
import { AccountContextProvider } from './common/contexts/AccountContext';

const router = createBrowserRouter([
  {
    path: '',
    element: <AppLayout routes={navigationRoutes} />,
    children: getRoutes(),
  },
]);

const root = ReactDOM.createRoot(
  document.getElementById('root') as HTMLElement
);
root.render(
  <React.StrictMode>
    <AccountContextProvider>
      <RouterProvider router={router} />
    </AccountContextProvider>
  </React.StrictMode>
);

// If you want to start measuring performance in your app, pass a function
// to log results (for example: reportWebVitals(console.log))
// or send to an analytics endpoint. Learn more: https://bit.ly/CRA-vitals
reportWebVitals();
