import React, { useState } from "react";
import { ReactNode } from "react";
import HeaderText from "./HeaderText";
import Icon from "./Icon";

export default function HiddenDiv({ title, children }: { title: string, children: ReactNode }) {
    const [shown, setShown] = useState<boolean>(false);

    return (
        <>
            <div onClick={() => setShown(!shown)} style={{ display: "flex", alignItems: "center" }}>
                <HeaderText>{title}</HeaderText>
                <Icon style={{ width: "40px" }} icon={shown ? "arrow_down" : "arrow_right"} />
            </div>
            {shown && children}
        </>
    )
}