import { createFileRoute, useNavigate } from '@tanstack/react-router';
import useApi from '@lib/providers/useApi';
import { Button } from '@components/ui/button';
import { IconChevronLeft } from '@tabler/icons-react';

export const Route = createFileRoute('/app/movie/$movieId')({
  component: MovieDesctiptionScreen,
});

function MovieDesctiptionScreen() {
  const { movieId } = Route.useParams();

  const navigate = useNavigate();
  const api = useApi();

  const { data: movie_data } = api.useQuery('get', '/api/movies/{movie_id}', {
    params: {
      //@ts-expect-error
      path: { movie_id: movieId },
    },
  });

  return (
    <div className="w-full ">
      <div className="aspect-[16/7.5]  w-full overflow-hidden relative">
        <div className="w-full h-full relative">
          {movie_data?.background_url && (
            <img className="w-full h-full object-cover" src={movie_data?.background_url} />
          )}
          <div className="absolute top-0 left-0 right-0 bottom-0 bg-gradient-to-t from-background to-transparent" />
          <div className="absolute left-0 top-0 flex p-5 ">
            <Button variant="outline" size="icon" onClick={() => navigate({ to: '/app/home' })}>
              <IconChevronLeft size={24} />
            </Button>
          </div>
          <div className="absolute left-0 right-0 bottom-0 flex justify-between items-end p-5 px-7">
            <div className="max-w-full flex md:max-w-1/2 flex-col items-start justify-end z-2 gap-3">
              <div className="text-5xl font-bold">{movie_data?.name}</div>
              <div className="text-lg text-muted-foreground line-clamp-5">
                {movie_data?.description}
              </div>
            </div>
          </div>
        </div>
      </div>
      <div className="w-full max-h-fit flex flex-col p-10 items-center gap-4">
        <div className="w-full flex items-center gap-2">
          <div className="text-xl font-bold">Genres:</div>
          <div className="flex flex-wrap gap-2 justify-center items-center">
            {movie_data?.genres && movie_data?.genres?.length > 0 ? (
              movie_data?.genres.map((tag) => (
                <div key={tag._id} className="bg-amber-100 rounded-2xl p-2">
                  <div className="text-md font-semibold text-black">{tag.name}</div>
                </div>
              ))
            ) : (
              <div className="text-md font-semibold text-muted-foreground">No genres available</div>
            )}
          </div>
        </div>
        <div className="w-full flex items-center gap-2">
          <div className="text-xl font-bold">Release date:</div>
          <div className="text-xl font-semibold">
            {movie_data?.release_date
              ? new Date(movie_data?.release_date).toLocaleDateString()
              : 'N/A'}
          </div>
        </div>
      </div>
    </div>
  );
}
