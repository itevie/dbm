

import { createSlice, PayloadAction } from "@reduxjs/toolkit";
import { Bot } from "../types/structures";

const initialState: { [key: string]: Bot } = {};

export const botsSlice = createSlice({
    name: "bots",
    initialState,
    reducers: {
        addBot: (state, action: PayloadAction<Bot>) => {
            return {
                ...state,
                [action.payload.id]: action.payload,
            }
        }
    }
});

export const { addBot } = botsSlice.actions;