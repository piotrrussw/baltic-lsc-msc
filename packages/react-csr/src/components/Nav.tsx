import { Link } from "react-router-dom";
import styled from "styled-components";

const Bar = styled.nav`
  --accent-color: #24a0ed;

  font-size: 18px;
  border: 1px solid var(--accent-color);
  padding-bottom: 10px;
  margin: 0;

  @media (min-width: 768px) {
    display: flex;
    justify-content: space-between;
    padding-bottom: 0;
    height: 70px;
    align-items: center;
  }
`;

const MainNav = styled.ul`
  list-style-type: none;
  flex-direction: column;
  margin-left: 0;
  padding-left: 0;

  @media (min-width: 768px) {
    display: flex;
    flex-direction: row;
    justify-content: flex-end;
  }
`;

const NavLi = styled.li`
  text-align: center;
  margin: 15px auto;
`;

const NavLink = styled(Link)`
  list-style-type: none;
  display: flex;
  flex-direction: column;
  @media (min-width: 768px) {
    margin: 0px 10px;
  }
`;

export function Nav() {
  return (
    <Bar>
      <MainNav>
        <NavLi>
          <NavLink to="/">Store</NavLink>
        </NavLi>
        <NavLi>
          <NavLink to="/shelf">Shelf</NavLink>
        </NavLi>
        <NavLi>
          <NavLink to="/app/test">App</NavLink>
        </NavLi>
      </MainNav>
    </Bar>
  );
}
