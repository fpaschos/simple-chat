import React from 'react';
import { Provider } from 'react-redux';
import './App.css';
import { store } from './app/store';
import Channel from './channel/Channel';
import WebSocketProvider from './websocket/WebsocketContext';

const App: React.FC = () => {
  return (
    <Provider store={store}>
      <WebSocketProvider>
        <Channel></Channel>
      </WebSocketProvider>
    </Provider>
  );
}

export default App;
