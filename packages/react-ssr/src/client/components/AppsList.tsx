import { useMutation, useQueryClient } from "react-query";
import dayjs from "dayjs";
import { App, Filter, Shelf } from "../types";
import { RequestMethod, sendRequest } from "../api";
import { Link } from "react-router-dom";
import { useStore } from "../store/StoreProvider";

const dateSort = (a: App, b: App) => {
  const dateA = a.releases?.[a?.releases?.length - 1]?.date;
  const dateB = b.releases?.[b?.releases?.length - 1]?.date;

  const timestampA = dateA ? new Date(dateA).getTime() : Number.MAX_VALUE;
  const timestampB = dateB ? new Date(dateB).getTime() : Number.MAX_VALUE;

  return timestampA > timestampB ? 1 : -1;
};

const nameSort = (a: App, b: App) => (a.name > b.name ? 1 : -1);

const getFilteredData = (apps: App[], shelf: Shelf[], filter: Filter) => {
  const searchValue = (filter.search || "").toLowerCase();
  const data = apps
    .map((app) => {
      const inCockpit = shelf.some((s) => s.unit.uid === app.uid);
      return { ...app, inCockpit };
    })
    .filter(
      (d) => d.name?.toLowerCase()?.includes(searchValue) || d.shortDescription?.toLowerCase()?.includes(searchValue)
    );

  data.sort((a, b) => (filter.sort === "date" ? dateSort(a, b) : nameSort(a, b)));

  return data;
};

export function AppsList() {
  const { state } = useStore();
  const queryClient = useQueryClient();

  const { mutate: addToCockpit, isLoading: isAddToCockpitLoading } = useMutation(
    (uid: string) => sendRequest(RequestMethod.POST, `/app/shelf?releaseUid=${uid}`, {}),
    {
      onSuccess: () => {
        queryClient.invalidateQueries({ queryKey: ["/shelf"] });
      },
    }
  );

  const { mutate: removeFromCockpit, isLoading: isRemoveFromCockpitLoading } = useMutation(
    (uid: string) => sendRequest(RequestMethod.DELETE, `/app/shelf?releaseUid=${uid}`),
    {
      onSuccess: () => {
        queryClient.invalidateQueries({ queryKey: ["/shelf"] });
      },
    }
  );

  const toggleCockpit = (app: App) => {
    if (isAddToCockpitLoading || isRemoveFromCockpitLoading) {
      return null;
    }

    const uid = app.releases[app.releases.length - 1].uid;
    app.inCockpit ? removeFromCockpit(uid) : addToCockpit(uid);
  };

  return (
    <div className="row">
      {getFilteredData(state.apps, state.shelf, state.filters).map((item) => {
        const lastUpdate = item.releases?.[item?.releases?.length - 1]?.date;

        return (
          <div key={item.uid} className="col col-md-4">
            <div className="card card-max-height">
              <div className="row inline-card-title">
                <img src={item.icon} width={50} height="auto" />
                <div className="card-title">
                  <Link to={`/app/${item.uid}`}>{item.name}</Link>
                </div>
              </div>
              <p>{item.shortDescription}</p>
              {lastUpdate && (
                <div>
                  <strong>Updated on:</strong> {dayjs(lastUpdate).format("YYYY-MM-DD HH:mm")}
                </div>
              )}
              <div className="card-actions">
                <button
                  className={`${
                    item.inCockpit ? "button-primary-outlined" : "button-primary"
                  } button-small button-round`}
                  onClick={() => toggleCockpit(item)}
                >
                  {item.inCockpit ? "In cockpit" : "Add to cockpit"}
                </button>
              </div>
            </div>
          </div>
        );
      })}
    </div>
  );
}

export const AppsListSkeleton = () => {
  return (
    <div className="row">
      <div className="col col-md-4">
        <div className="card card-max-height skeleton" />
      </div>
      <div className="col col-md-4">
        <div className="card card-max-height skeleton" />
      </div>
      <div className="col col-md-4">
        <div className="card card-max-height skeleton" />
      </div>
    </div>
  );
};