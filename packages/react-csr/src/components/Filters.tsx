import { ChangeEvent } from "react";
import { useAtom } from "jotai";
import styled from "styled-components";
import { filtersAtom } from "../store";

const Wrapper = styled.div`
  padding: 1rem 0;
  font-size: 1.75rem;
  font-weight: 500;
  color: rgb(36, 41, 46);

  & a {
    font-size: 1rem;
    padding-top: 2rem;
    color: rgb(106, 115, 125);
  }
`;

const Header = styled.div`
  font-size: 1.75rem;
  font-weight: 500;
  color: rgb(36, 41, 46);

  * {
    margin: 0 1rem;
  }
`;

const Input = styled.input`
  width: 300px;
  font-size: 15px;
  font-weight: 500;
  line-height: 1;
  text-align: left;
  color: #6190ff;
  background-color: transparent;
  border: 1px solid rgba(0, 118, 255, 0.1);
  padding: 10px 7px;
  border-radius: 5px;
  outline: none;
  border: 0px;
  border-radius: 0px;
  transition: all 0.2s ease-in-out 0s;
  border: 1px solid rgba(0, 118, 255, 0.1);
  border-radius: 6px;
`;

const Select = styled.select`
  width: 100px;
  font-size: 15px;
  font-weight: 500;
  line-height: 1;
  text-align: left;
  color: #6190ff;
  background-color: transparent;
  border: 1px solid rgba(0, 118, 255, 0.1);
  padding: 10px 7px;
  border-radius: 5px;
  outline: none;
  cursor: pointer;
  border: 0px;
  border-radius: 0px;
  transition: all 0.2s ease-in-out 0s;
  border: 1px solid rgba(0, 118, 255, 0.1);
  border-radius: 6px;

  & option {
    font-size: 14px;
    font-weight: 500;
    padding: 10px 7px;
    border-radius: 5px;
    background-color: #f8f9ff;
  }
`;

export const Filters = () => {
  const [filter, setFilter] = useAtom(filtersAtom);

  const handleSort = (event: ChangeEvent<HTMLSelectElement>) => {
    setFilter({ ...filter, sort: event.target.value });
  };

  const handleInput = (event: ChangeEvent<HTMLInputElement>) => {
    setFilter({ ...filter, search: event.target.value });
  };

  return (
    <Wrapper>
      <Header>
        <Input type="search" onChange={handleInput} placeholder="Search app by name or description" />
        <Select onChange={handleSort} value={filter.sort}>
          <option value="date">Date</option>
          <option value="name">Name</option>
        </Select>
      </Header>
    </Wrapper>
  );
};
