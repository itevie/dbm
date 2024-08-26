import React, { HTMLAttributes } from "react";
import { ReactNode } from "react";

export default function Container(props: { children: ReactNode } & HTMLAttributes<HTMLDivElement>) {
    return (
        <div {...props} className={`container ${props.className ?? ""}`}>
            {props.children}
        </div>
    )
}