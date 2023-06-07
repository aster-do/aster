import React from 'react';
import ReactDOM from 'react-dom/client';
import { RouterProvider, createBrowserRouter } from 'react-router-dom';
import { LocalizationProvider } from '@mui/x-date-pickers';
import { AdapterDayjs } from '@mui/x-date-pickers/AdapterDayjs';
import reportWebVitals from './reportWebVitals';
import AppLayout from './common/components/AppLayout';
import getRoutes, { navigationRoutes } from './common/navigation';
import { AccountContextProvider } from './common/contexts/AccountContext';

import '@carbon/styles/css/styles.css';
import '@carbon/charts/styles.css';
import './index.css';

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
    <LocalizationProvider dateAdapter={AdapterDayjs}>
      <AccountContextProvider>
        <RouterProvider router={router} />
      </AccountContextProvider>
    </LocalizationProvider>
  </React.StrictMode>
);

// If you want to start measuring performance in your app, pass a function
// to log results (for example: reportWebVitals(console.log))
// or send to an analytics endpoint. Learn more: https://bit.ly/CRA-vitals
reportWebVitals();
