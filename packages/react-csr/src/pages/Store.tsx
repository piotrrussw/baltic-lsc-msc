import { useAtom } from "jotai";
import { useQuery } from "react-query";
import { sendRequest, RequestMethod } from "../api";
import { appsAtom, shelfAtom } from "../store";
import { App, Shelf } from "../types";
import { AppsList, AppsListSkeleton } from "../components/AppsList";
import { Filters } from "../components/Filters";

export function Store() {
  const [apps, setApps] = useAtom(appsAtom);
  const [shelf, setShelf] = useAtom(shelfAtom);

  const payload = {
    onlyApps: true,
    onlyLastRelease: false,
  };

  const { isLoading: isAppLoading } = useQuery<App[]>(
    ["/list", payload],
    () => sendRequest(RequestMethod.POST, "/list", payload),
    {
      onSuccess: (response) => {
        const fetchedApps = response.filter((a) => a.releases.length > 0);
        fetchedApps.sort((a, b) => new Date(a.releases[0].date).getTime() - new Date(b.releases[0].date).getTime());

        setApps(fetchedApps);
      },
      cacheTime: 300_000,
      staleTime: 300_000,
    }
  );

  const { isLoading: isShelfLoading } = useQuery<Shelf[]>(["/shelf"], () => sendRequest(RequestMethod.GET, "/shelf"), {
    onSuccess: (response) => {
      setShelf(response);
    },
    cacheTime: 300_000,
    staleTime: 300_000,
  });

  const isLoading = isShelfLoading || isAppLoading;

  return (
    <main>
      <Filters />
      {isLoading ? <AppsListSkeleton /> : <AppsList />}
    </main>
  );
}
