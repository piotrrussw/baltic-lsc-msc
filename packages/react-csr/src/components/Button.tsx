import styled, { css } from "styled-components";

export const Button = styled.a<{ primary?: boolean }>`
  --accent-color: #24a0ed;

  background: transparent;
  border-radius: 3px;
  border: 1px solid var(--accent-color);
  background-color: var(--accent-color);
  color: white;
  display: inline-block;
  margin: 0.5rem 1rem;
  padding: 0.5rem 0;
  transition: all 200ms ease-in-out;
  width: 11rem;
  display: flex;
  justify-content: center;
  cursor: pointer;

  &:hover {
    filter: brightness(0.85);
  }

  &:active {
    filter: brightness(1);
  }

  ${(props) =>
    props.primary &&
    css`
      background: white;
      color: var(--accent-color);
    `}
`;
