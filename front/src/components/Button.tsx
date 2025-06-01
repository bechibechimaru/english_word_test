type ButtonProps = {
    type?: "button" | "submit" | "reset";
    onClick?: () => void;
    disabled?: boolean;
    className?: string;
    children?: React.ReactNode;
};

function Button({ type = "button", onClick, disabled = false, className, children }: ButtonProps) {
    return (
        <button 
            type={type} 
            onClick={onClick} 
            disabled={disabled}
            className={className}
        >
            {children}
        </button>
    );
}

export default Button;
