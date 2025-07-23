import { MouseEventHandler } from 'react';
import { IconChevronLeft, IconChevronRight } from '@tabler/icons-react';
import { Button } from '@components/ui/button';

interface CarouselControlsProps {
    onLeftClick: MouseEventHandler<HTMLButtonElement>;
    onRightClick: MouseEventHandler<HTMLButtonElement>;
}

export default function Controls({ onLeftClick, onRightClick }: CarouselControlsProps) {
    return (
        <div className="flex gap-4 items-center justify-center h-full ">
            <Button size={'icon'} onClick={onLeftClick} variant={'outline'}>
                <IconChevronLeft />
            </Button>
            <Button size={'icon'} onClick={onRightClick} variant={'outline'}>
                <IconChevronRight />
            </Button>
            {/* <div
        onClick={onLeftClick}
        className="cursor-pointer z-10 flex items-center justify-center"
      >
        <IconChevronLeft size={50} color="#fff" />
      </div> */}
            {/* <div
        onClick={onRightClick}
        className="cursor-pointer z-10 flex items-center justify-center"
      >
        <IconChevronRight size={50} color="#fff" />
      </div> */}
        </div>
    );
}
