

import { createSlice, PayloadAction } from "@reduxjs/toolkit";
import { Command } from "../types/structures";

const initialState: { [key: string]: Command } = {};

export const commandsSlice = createSlice({
    name: "commands",
    initialState,
    reducers: {
        addCommand: (state, action: PayloadAction<Command>) => {
            return {
                ...state,
                [action.payload.id]: action.payload,
            }
        }
    }
});

export const { addCommand } = commandsSlice.actions;