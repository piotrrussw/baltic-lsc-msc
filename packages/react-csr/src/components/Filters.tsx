import { ChangeEvent } from "react";
import { useAtom } from "jotai";
import { filtersAtom } from "../store";

export const Filters = () => {
  const [filter, setFilter] = useAtom(filtersAtom);

  const handleSort = (event: ChangeEvent<HTMLSelectElement>) => {
    setFilter({ ...filter, sort: event.target.value });
  };

  const handleInput = (event: ChangeEvent<HTMLInputElement>) => {
    setFilter({ ...filter, search: event.target.value });
  };

  return (
    <div className="row py-10">
      <div className="col col-sm-6">
        <div className="form-control">
          <input type="search" onChange={handleInput} placeholder="Search app by name or description" />
        </div>
      </div>

      <div className="col col-sm-2">
        <div className="form-control">
          <select onChange={handleSort} value={filter.sort}>
            <option value="date">Date</option>
            <option value="name">Name</option>
          </select>
        </div>
      </div>
    </div>
  );
};
