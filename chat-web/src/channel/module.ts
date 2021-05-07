import { ClientMessageType, ID, Message } from "../api/types";
import { Action } from "../app/store";

// Actions
export enum ChannelActionType {
    ChannelMessage = 'channel/message',
    SendMessage = 'channel/send-message',


}
export function chatMessage(user: ID, msg: Message): Action {
    return {
        type: ChannelActionType.ChannelMessage,
        payload: { user, msg },
    }
}

export function sendMessage(user: ID, msg: Message): Action {
    return {
        type: ChannelActionType.SendMessage,
        payload: { user, msg },
    }
}

// State
export type InitialState = {
    user: string,
    name: string,
    messages: string[],
};

const initialState = {
    user: "",
    name: "#Channel 1",
    messages: [],
};



export const channelReducer = (state: InitialState = initialState, action: Action): InitialState => {
    return initialState
}
