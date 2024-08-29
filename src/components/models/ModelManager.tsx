import React, { ReactNode, useEffect, useState } from "react";
import Button from "../Button";
import Input from "../Input";

export interface ModelButtonInstanceClickData {
    close: () => void,
    inputData: { [key: string]: string },
}

export interface ModelButtonInstance {
    name: string,
    displayName?: string,
    onClick: (data: ModelButtonInstanceClickData) => void,
    type: "primary" | "secondary" | "error" | "success",
    key?: "esc" | "enter"
}

export interface ModelInputInstance {
    name: string,
    title: string
}

export interface ModelInstance {
    title: string,
    body?: ReactNode,
    buttons?: ModelButtonInstance[],
    inputs?: ModelInputInstance[],
}

export const modelQueue: ModelInstance[] = [];

export let pushModel: (model: ModelInstance) => void = () => { };
export let popModel: () => void = () => { };

export default function ModelManager() {
    const [currentModel, setCurrentModel] = useState<ModelInstance | null>(null);
    const [currentInputData, setCurrentInputData] = useState<{ [key: string]: string }>({});

    document.addEventListener("keydown", e => {
        if (!currentModel) return;

        if (e.key === "Escape") {
            for (const button of currentModel.buttons ?? []) {
                if (button.key === "esc") {
                    button.onClick({
                        close: () => popModel(),
                        inputData: currentInputData
                    });
                }
            }
        } else if (e.key === "Enter") {
            for (const button of currentModel.buttons ?? []) {
                if (button.key === "enter") {
                    button.onClick({
                        close: () => popModel(),
                        inputData: currentInputData
                    });
                }
            }
        }
    });

    useEffect(() => {
        const updateModel = () => {
            setCurrentInputData({});
            setCurrentModel(modelQueue[modelQueue.length - 1]);
        };

        pushModel = model => {
            modelQueue.push(model);
            updateModel();
        };

        popModel = () => {
            modelQueue.pop();
            updateModel();
        }
    }, []);

    function buttonClicked(button: ModelButtonInstance) {
        const data: ModelButtonInstanceClickData = {
            close: () => popModel(),
            inputData: currentInputData
        };

        button.onClick(data);
    }

    return (
        <>
            {!currentModel ? <></>
                :
                <div className="model-instance-manager">
                    <div className="instance-model">
                        <label className="model-header">{currentModel?.title}</label>
                        <p style={{ display: "block", width: "100%" }}>
                            {currentModel?.body}
                        </p>
                        {
                            currentModel?.inputs?.map(input => <>
                                <label>{input.title}</label>
                                <Input className="jumbo" onChange={(i) => setCurrentInputData({ ...currentInputData, [input.name]: (i.target as any).value })} />
                            </>)
                        }
                        <div className="side-by-side">
                            {
                                currentModel?.buttons?.map(button => <>
                                    <Button className="jumbo" onClick={() => buttonClicked(button)} type={button.type}>{button.displayName ?? button.name}</Button>
                                </>)
                            }
                        </div>
                    </div>
                </div>
            }
        </>
    )
}