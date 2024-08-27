import React, { HTMLAttributes, ReactNode } from "react";

const Input = React.forwardRef((props: HTMLAttributes<HTMLInputElement>, ref: React.ForwardedRef<HTMLInputElement>) => {
    return (
        <input ref={ref} {...props} className={`input ${props.className}`} />
    );
});

export default Input;