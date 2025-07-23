import { createFileRoute } from '@tanstack/react-router';

export const Route = createFileRoute('/app/movie/$movieId')({
    component: RouteComponent,
});

function RouteComponent() {
    return <div className="aspect-[16/7.5]  w-full overflow-hidden relative"></div>;
}
