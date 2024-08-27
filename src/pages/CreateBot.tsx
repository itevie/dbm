import React, { useRef } from "react";
import HeaderText from "../components/HeaderText";
import Button from "../components/Button";
import { invoke } from "@tauri-apps/api";
import { setSettings } from "../stores/options";
import { useDispatch } from "react-redux";
import { addBot } from "../stores/bots";
import { Bot } from "../types/structures";
import Input from "../components/Input";

export default function CreateBot() {
    const dispatch = useDispatch();
    const nameRef = useRef<HTMLInputElement>(null);
    const tokenRef = useRef<HTMLInputElement>(null);


    async function createBot() {
        let name = nameRef.current?.value;
        let token = tokenRef.current?.value;

        console.log(name, token);

        let result = await invoke("create_bot", { name: nameRef.current?.value, token: tokenRef.current?.value }) as Bot;
        dispatch(addBot(result));
        dispatch(setSettings({ current_bot: result.id }));
    }

    return (
        <>
            <HeaderText>Create a new bot</HeaderText>
            <label>Name:</label>
            <Input className="jumbo" ref={nameRef} />
            <label>Token:</label>
            <Input className="jumbo" ref={tokenRef} />
            <Button className="jumbo" onClick={createBot}>Create</Button>
        </>
    );
}