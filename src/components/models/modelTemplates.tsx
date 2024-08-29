import { ReactNode } from "react";
import { pushModel } from "./ModelManager";

export function asyncBasicInputModel(name: string, body: ReactNode = "", inputTitle: string = ""): Promise<string | null> {
    return new Promise<string | null>(resolve => {
        basicInputModel(name, body, result => {
            if (!result) resolve(null);
            else resolve(result);
        }, inputTitle);
    });
}

export function basicInputModel(name: string, body: ReactNode = "", onSubmit: (result: string) => void, inputTitle: string = "") {
    pushModel({
        title: name,
        body,
        inputs: [
            {
                name: "result",
                title: inputTitle
            }
        ],
        buttons: [
            {
                name: "cancel",
                type: "error",
                onClick: d => {
                    d.close();
                },
                key: "esc"
            },
            {
                name: "submit",
                type: "primary",
                onClick: d => {
                    onSubmit(d.inputData["result"] || "");
                    d.close();
                },
                key: "enter"
            }
        ]
    })
}

export function errorMessage(body: ReactNode, title: string = "Error") {
    pushModel({
        title,
        body,
        buttons: [
            {
                type: "primary",
                name: "okay",
                onClick: d => {
                    d.close();
                }
            }
        ]
    });
}

export function messageModel(body: ReactNode, title: string = "Info") {
    pushModel({
        title,
        body,
        buttons: [
            {
                type: "primary",
                name: "okay",
                onClick: d => {
                    d.close();
                }
            }
        ]
    });
}