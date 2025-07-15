import { cn } from '@lib/utils';

export type SplashSectionProps = React.ComponentProps<'div'> & {
    children?: React.ReactNode;
};

export function SplashSection({ children, className }: SplashSectionProps) {
    return (
        <div
            className={cn(
                'flex w-screen h-screen items-center justify-center bg-background',
                className
            )}
        >
            <div className="flex flex-col items-center z-2">{children}</div>
        </div>
    );
}
