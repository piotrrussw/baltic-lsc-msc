import { useParams } from "react-router-dom";
import { useQuery } from "react-query";
import dayjs from "dayjs";

import { App as AppType } from "../types";

import { getApp } from "../api";

const releaseStatusMap: Record<number, string> = {
  0: "Created",
  2: "Approved",
};

export function App() {
  const { id } = useParams<{ id: string }>();

  const path = `?appUid=${id}`;
  const { data, isLoading } = useQuery<AppType>([path], getApp(path), {
    cacheTime: 300_000,
    staleTime: 300_000,
  });

  if (isLoading || !data) {
    return <AppSkeleton />;
  }

  return (
    <div className="v-stack">
      <div className="panel full-width">
        <div className="panel-head panel-left">
          <img src={data.icon} width={50} height="auto" />
          <p className="panel-title">{data.name}</p>
        </div>
        <div className="panel-body">
          <p>{data.longDescription || "No description"}</p>
        </div>
        <div className="panel-footer">
          <ul className="tags">
            {data.isApp && <li className="tag tag-green">App</li>}
            {data.isService && <li className="tag tag-blue">Service</li>}
          </ul>
        </div>
      </div>

      <div className="panel full-width">
        <div className="panel-head panel-left">
          <p className="panel-title">Releases</p>
        </div>
        <div className="panel-body pt-0">
          <table>
            <thead>
              <tr>
                <th>Version name</th>
                <th>Release date</th>
                <th>Description</th>
                <th>OpenSource</th>
                <th>Status</th>
              </tr>
            </thead>
            <tbody>
              {data.releases.map((release) => {
                return (
                  <tr key={release.uid}>
                    <td>
                      <p>
                        <strong>{release.version}</strong>
                      </p>
                    </td>
                    <td>
                      <p>{dayjs(release.date).format("YYYY-MM-DD HH:mm")}</p>
                    </td>
                    <td>
                      <p>{release.description || "-"}</p>
                    </td>
                    <td>
                      <p>{release.openSource ? "Yes" : "No"}</p>
                    </td>
                    <td>
                      <p>{releaseStatusMap[release.status] || "-"}</p>
                    </td>
                  </tr>
                );
              })}
            </tbody>
          </table>
        </div>
      </div>
    </div>
  );
}

const AppSkeleton = () => (
  <div className="row">
    <div className="panel" />
  </div>
);
