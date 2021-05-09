import { UserId, ID, Message } from "../api/types";
import { Action } from "../app/store";

// Actions
export enum ChannelActionType {
    ChangeUser = 'channel/change-user',
    ChannelMessage = 'channel/message',
    SendMessage = 'channel/send-message',
}

export type ChannelAction = ChangeUserAction | ChannelMessageAction | SendMessageAction;
export interface ChangeUserAction extends Action {
    payload: { user: UserId }
}

export interface ChannelMessageAction extends Action {
    payload: { msg: Message }
}

export interface SendMessageAction extends Action {
    payload: { user: UserId, msg: string }
}

export function changeUser(user: UserId): ChangeUserAction {
    return {
        type: ChannelActionType.ChangeUser,
        payload: { user },
    }
}

export function channelMessage(msg: Message): ChannelMessageAction {
    return {
        type: ChannelActionType.ChannelMessage,
        payload: { msg },
    }
}

export function sendMessage(user: ID, msg: string): SendMessageAction {
    return {
        type: ChannelActionType.SendMessage,
        payload: { user, msg },
    }
}

// State
export type InitialState = {
    user: UserId,
    name: string, // Channel name
    channel_id: ID,
    messages: Message[],
};

const initialState = {
    user: "",
    name: "#Channel 1", // Hardcoded for now
    channel_id: "",
    messages: [],
};



export const channelReducer = (state: InitialState = initialState, action: ChannelAction): InitialState => {
    switch (action.type) {
        case ChannelActionType.ChangeUser: {
            let a = action as ChangeUserAction
            let { user } = a.payload
            return { ...state, user }
        }
        case ChannelActionType.SendMessage: {
            return state
        }
        case ChannelActionType.ChannelMessage: {
            let a = action as ChannelMessageAction
            let { msg } = a.payload
            let messages = [...state.messages, msg]
            let channel_id = msg.channel_id
            return {
                ...state, channel_id, messages
            }
        }
    }
    return state
}
