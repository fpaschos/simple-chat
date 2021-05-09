import React, { useContext, useState } from 'react';
import { shallowEqual, useDispatch, useSelector } from 'react-redux';
import { ClientMessageType, SendMessage } from '../api/types';
import { RootState } from '../app/store';
import { WebSocketContext } from '../websocket/WebsocketContext';
import { changeUser } from './module';

const Channel: React.FC = () => {
    // Redux state
    const channel = useSelector((s: RootState) => s.channel, shallowEqual)
    const dispatch = useDispatch()

    // Websocket access
    const ws = useContext(WebSocketContext)

    // Internal component state
    const [message, setMessage] = useState("");
    const [userId, setUserId] = useState("");

    // Handle message input changes
    const onMessageChange = (ev: React.ChangeEvent<HTMLInputElement>) => {
        setMessage(ev.target.value);
    }

    const onMessageKeyUp = (ev: React.KeyboardEvent<HTMLInputElement>) => {
        if (ev.code === "Enter") {
            sendMessage(message)
        }
    }

    const sendMessage = (content: string) => {
        let m = content.trim()
        if (m !== "") {
            let msg: SendMessage = { type: ClientMessageType.SendMessage, user: channel.user, content }
            ws?.sendMessage(msg)
            setMessage("") // Clearing the message on enter
        }
    }

    // Handle user id changes
    const onUserIdChange = (ev: React.ChangeEvent<HTMLInputElement>) => {
        setUserId(ev.target.value);
    }

    const onUserIdKeyUp = (ev: React.KeyboardEvent<HTMLInputElement>) => {
        if (ev.code === "Enter") {
            changeUserId(userId)
        }
    }

    const changeUserId = (userId: string) => {
        let u = userId.trim()
        if (u !== "") {
            dispatch(changeUser(userId))
            setUserId("") // Clearing the user id on enter
        }
    }

    return (
        <>
            <h3>Channel {channel.name} {channel.user} </h3>
            <div>
                <p>Set your user id:</p>
                <input id="user-id" type="text"
                    value={userId}
                    onKeyUp={(ev) => onUserIdKeyUp(ev)}
                    onChange={(ev) => onUserIdChange(ev)} />
                <button onClick={() => changeUserId(userId)}>{'>'}</button>
            </div>
            <div>
                {channel.messages.map(m => <div key={m.id}>
                    {m.sender} - {m.content}
                </div>)}
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