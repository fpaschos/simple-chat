import { send } from 'process';
import React, { useContext, useState } from 'react';
import { shallowEqual, useSelector } from 'react-redux';
import { RootState } from '../app/store';
import { WebSocketContext } from '../websocket/WebsocketContext';

const Channel: React.FC = () => {
    const channel = useSelector((s: RootState) => s.channel, shallowEqual)
    const [message, setMessage] = useState("");
    const ws = useContext(WebSocketContext)

    const onMessageChange = (ev: React.ChangeEvent<HTMLInputElement>) => {
        setMessage(ev.target.value);
    }

    const onMessageKeyUp = (ev: React.KeyboardEvent<HTMLInputElement>) => {
        if (ev.code === "Enter") {
            sendMessage(message)
        }
    }

    const sendMessage = (message: string) => {
        let m = message.trim()
        if (m !== "") {
            console.log("Sending " + message)
            ws?.sendMessage(message)
            setMessage("") // Clearing the message on enter
        }
    }

    return (
        <>
            <h3>Channel {channel.name} </h3>
            <div>
                {channel.messages.map(m => <div>{m}</div>)}
            </div>
            <div>
                <input id="message" type="text"
                    value={message}
                    onKeyUp={(ev) => onMessageKeyUp(ev)}
                    onChange={(ev) => onMessageChange(ev)} />

                <button onClick={() => sendMessage(message)}>{'>'}</button>
            </div>
        </>
    );
};

export default Channel;