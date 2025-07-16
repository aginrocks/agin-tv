import { MouseEventHandler } from "react";
import { IconChevronLeft, IconChevronRight } from "@tabler/icons-react";

interface CarouselControlsProps {
  onLeftClick?: MouseEventHandler<HTMLDivElement>;
  onRightClick?: MouseEventHandler<HTMLDivElement>;
}

export default function Controls({
  onLeftClick,
  onRightClick,
}: CarouselControlsProps) {
  return (
    <div className="flex gap-4 items-center justify-center h-full p-10">
      <div
        onClick={onLeftClick}
        className="cursor-pointer z-10 flex items-center justify-center"
      >
        <IconChevronLeft size={50} color="#fff" />
      </div>
      <div
        onClick={onRightClick}
        className="cursor-pointer z-10 flex items-center justify-center"
      >
        <IconChevronRight size={50} color="#fff" />
      </div>
    </div>
  );
}
