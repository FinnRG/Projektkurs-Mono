import React from 'react';
import ReactDOM from 'react-dom';
import './index.css';
import WrappedContext from './WrappedContext';

ReactDOM.render(
  <React.StrictMode>
    <WrappedContext />
  </React.StrictMode>,
  document.getElementById('root')
);
