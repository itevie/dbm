import React, { HTMLAttributes, ReactNode } from "react";

export default function Button(props: { children: ReactNode } & HTMLAttributes<HTMLButtonElement>) {
    return (
        <button {...props} className={`button ${props.className}`}>
            {props.children}
        </button>
    );
}