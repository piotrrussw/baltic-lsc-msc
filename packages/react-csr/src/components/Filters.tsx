import { ChangeEvent } from "react";
import { useStore } from "../store/StoreProvider";

export const Filters = () => {
  const { state, dispatch } = useStore();

  const handleSort = (event: ChangeEvent<HTMLSelectElement>) => {
    dispatch({ type: "SET_FILTERS", payload: { ...state.filters, sort: event.target.value } });
  };

  const handleInput = (event: ChangeEvent<HTMLInputElement>) => {
    dispatch({ type: "SET_FILTERS", payload: { ...state.filters, search: event.target.value } });
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
          <select onChange={handleSort} value={state.filters.sort}>
            <option value="date">Date</option>
            <option value="name">Name</option>
          </select>
        </div>
      </div>
    </div>
  );
};
