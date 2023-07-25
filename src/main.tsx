import React from 'react'
import ReactDOM from 'react-dom/client'
import App from './App.tsx'
import { BrowserRouter, Routes, Route } from "react-router-dom";
import './styles/index.css'
import './styles/main.scss'
import Login from './Login.tsx';

ReactDOM.createRoot(document.getElementById('root') as HTMLElement).render(
  <React.StrictMode>
    <BrowserRouter>
      <Routes>
        <Route path="/">
          <Route index element={<App />} />
          <Route path='/sign-up' element={<Login />} />
        </Route>
      </Routes>
    </BrowserRouter>
  </React.StrictMode>,
)
