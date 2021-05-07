import { combineReducers } from "redux"
import { createStore } from "redux"
import { channelReducer, InitialState as ChannelState } from "../channel/module"

export type Action = {
    type: string,
    payload: any,
}

export type RootState = {
    channel: ChannelState,
}

const rootReducer = combineReducers({
    channel: channelReducer,
})

export const store = createStore(rootReducer)