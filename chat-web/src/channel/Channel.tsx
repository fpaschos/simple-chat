import React, { useState } from 'react';
import { shallowEqual, useSelector } from 'react-redux';
import { RootState } from '../app/store';

const Channel: React.FC = () => {
    const channel = useSelector((s: RootState) => s.channel, shallowEqual)
    const [message, setMessage] = useState("");

    const onMessageChange = (ev: React.ChangeEvent<HTMLInputElement>) => {
        setMessage(ev.target.value);
    }

    const onMessageKeyUp = (ev: React.KeyboardEvent<HTMLInputElement>) => {
        if (ev.code === "Enter") {
            console.log("Sending " + message)
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
                <label htmlFor="message">Message:</label>
                <input id="message" type="text"
                    value={message}
                    onKeyUp={(ev) => onMessageKeyUp(ev)}
                    onChange={(ev) => onMessageChange(ev)} />

                <button>{'>'}</button>
            </div>
        </>
    );
};

export default Channel;