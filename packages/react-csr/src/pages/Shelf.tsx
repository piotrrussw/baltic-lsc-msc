import { useQuery } from "react-query";

import { GET_DATA_SHELF_QUERY_KEY, getDataShelf } from "../api";
import { DataShelf } from "../types";

const prettyJSON = (str: string) => {
  try {
    return JSON.stringify(JSON.parse(str), undefined, 2);
  } catch {
    return str;
  }
};

export function Shelf() {
  const { isLoading, data } = useQuery<DataShelf[]>([GET_DATA_SHELF_QUERY_KEY], getDataShelf, {
    cacheTime: 300_000,
    staleTime: 300_000,
  });

  if (isLoading || !data) {
    return <ShelfSkeleton />;
  }

  return (
    <div className="panel full-width">
      <div className="panel-head panel-left">
        <p className="panel-title">Data sets</p>
      </div>
      <div className="panel-body pt-0">
        <table>
          <thead>
            <tr>
              <th>Name</th>
              <th>Edit</th>
              <th>Multiplicity</th>
              <th>Data Type</th>
              <th>Data Structure</th>
              <th>Access Type</th>
              <th>Access values</th>
              <th>Path values</th>
              <th>Delete</th>
            </tr>
          </thead>

          <tbody>
            {data.map((shelf) => {
              return (
                <tr>
                  <td>
                    <p>
                      <strong>{shelf.name || "-"}</strong>
                    </p>
                  </td>
                  <td>
                    <button className="button-primary button-small button-round">Edit</button>
                  </td>
                  <td>
                    <p>{shelf.multiplicity > 0 ? "Multiple" : "Single"}</p>
                  </td>
                  <td>
                    <p>{shelf.dataTypeName || "-"}</p>
                  </td>
                  <td>
                    <p>{shelf.dataStructureName || "-"}</p>
                  </td>
                  <td>
                    <p>{shelf.accessTypeName || "-"}</p>
                  </td>
                  <td>
                    <pre>{shelf.accessValues ? prettyJSON(shelf.accessValues) : "-"}</pre>
                  </td>
                  <td>
                    <pre>{shelf.values ? prettyJSON(shelf.values) : "-"}</pre>
                  </td>
                  <td>
                    <button className="button-primary button-small button-round">Delete</button>
                  </td>
                </tr>
              );
            })}
          </tbody>
        </table>
      </div>
    </div>
  );
}

const ShelfSkeleton = () => (
  <div className="row">
    <div className="panel" />
  </div>
);
