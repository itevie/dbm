

import { createSlice, PayloadAction } from "@reduxjs/toolkit";
import { Settings } from "../types/structures";

const initialState: Settings = { current_bot: null };

export const settingsSlice = createSlice({
    name: "settings",
    initialState,
    reducers: {
        setSettings: (state, action: PayloadAction<Settings>) => {
            return action.payload;
        }
    }
});

export const { setSettings } = settingsSlice.actions;