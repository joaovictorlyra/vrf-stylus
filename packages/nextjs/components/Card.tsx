import React from "react";
import Link from "next/link";

interface CardPropsWithLink {
  icon: React.ReactNode;
  description: React.ReactNode;
  linkHref: string;
  linkText: string;
  isDarkMode: boolean;
  children?: never;
  className?: string;
}

interface CardPropsWithChildren {
  children: React.ReactNode;
  className?: string;
  icon?: never;
  description?: never;
  linkHref?: never;
  linkText?: never;
  isDarkMode?: boolean;
}

type CardProps = CardPropsWithLink | CardPropsWithChildren;

export const Card: React.FC<CardProps> = props => {
  // If children prop exists, render as simple container
  if ("children" in props && props.children) {
    return <div className={`bg-base-100 rounded-lg shadow-lg p-6 ${props.className || ""}`}>{props.children}</div>;
  }

  // Otherwise render with link pattern
  const { icon, description, linkHref, linkText, isDarkMode, className } = props as CardPropsWithLink;

  return (
    <div
      className={`relative h-full rounded-3xl border-2 border-transparent p-4 text-center flex flex-col items-center justify-evenly max-w-md ${
        isDarkMode ? "gradient-border-red" : "gradient-border-light"
      } ${className || ""}`}
      style={{
        boxShadow: "0 0 0 3px transparent",
      }}
    >
      <div className="absolute top-0 left-1/2 -translate-x-1/2">
        <svg xmlns="http://www.w3.org/2000/svg" width="84" height="6" viewBox="0 0 84 6" fill="none">
          <path d="M41.3071 6L15.6728 6L0 0L84 0L69.02 6L41.3071 6Z" fill="#E3066E" />
        </svg>
      </div>
      <div>{icon}</div>
      <p className="text-sm">
        {description}
        <br />
        <Link href={linkHref} passHref className="underline underline-offset-4 font-semibold">
          {linkText}
        </Link>{" "}
        tab.
      </p>
    </div>
  );
};

export default Card;
