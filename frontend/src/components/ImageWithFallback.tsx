"use client";

import Image, { ImageProps } from "next/image";
import { useState } from "react";

type Props = ImageProps & {
  fallbackClassName?: string;
};

export default function ImageWithFallback({
  src,
  alt,
  fallbackClassName,
  ...rest
}: Props) {
  const [error, setError] = useState(false);

  if (error) {
    return (
      <div
        aria-label={alt}
        className={fallbackClassName ?? "rounded-full bg-gray-200"}
      />
    );
  }

  return (
    <Image
      src={src}
      alt={alt}
      onError={() => setError(true)}
      {...rest}
    />
  );
}
