import { useMutation, useQueryClient } from "react-query";
import styled from "styled-components";
import dayjs from "dayjs";
import { App, Filter, Shelf } from "../types";
import { Button } from "./Button";
import { useAtom } from "jotai";
import { appsAtom, filtersAtom, shelfAtom } from "../store";
import { RequestMethod, sendRequest } from "../api";
import { Link } from "react-router-dom";

const StyledList = styled.ul`
  list-style: none;
  display: flex;
  flex-wrap: wrap;
`;

const ListItem = styled.li`
  padding: 10px;
  border: 1px solid #ccc;
  margin: 5px 15px;
  width: 250px;
`;

const Description = styled.p`
  color: #888;
`;

const TitleContainer = styled.div`
  display: flex;
  align-items: center;
`;

const Title = styled(Link)`
  margin: 0 0 0 25px;
  color: #333;
  font-size: 1.3rem;
  text-decoration: none;
  font-weight: 600;
`;

const ImageContainer = styled.div`
  width: 50px;
  height: 50px;
`;

const dateSort = (a: App, b: App) => new Date(a.releases[0].date).getTime() - new Date(b.releases[0].date).getTime();
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
  const [apps] = useAtom(appsAtom);
  const [shelf] = useAtom(shelfAtom);
  const [filter] = useAtom(filtersAtom);
  const queryClient = useQueryClient();

  const { mutate: addToCockpit, isLoading: isAddToCockpitLoading } = useMutation(
    (uid: string) => sendRequest(RequestMethod.POST, `/shelf?releaseUid=${uid}`, {}),
    {
      onSuccess: () => {
        queryClient.invalidateQueries({ queryKey: ["/shelf"] });
      },
    }
  );

  const { mutate: removeFromCockpit, isLoading: isRemoveFromCockpitLoading } = useMutation(
    (uid: string) => sendRequest(RequestMethod.DELETE, `/shelf?releaseUid=${uid}`),
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
    <StyledList>
      {getFilteredData(apps, shelf, filter).map((item) => {
        const lastUpdate = item.releases[item.releases.length - 1].date;

        return (
          <ListItem key={item.uid}>
            <TitleContainer>
              <ImageContainer>
                <img src={item.icon} width="100%" height="auto" />
              </ImageContainer>
              <Title to={`/app/${item.uid}`}>{item.name}</Title>
            </TitleContainer>
            <Description>{item.shortDescription}</Description>
            {lastUpdate && <Description>Updated on {dayjs(lastUpdate).format("YYYY-MM-DD HH:mm")}</Description>}
            <Button onClick={() => toggleCockpit(item)} primary={item.inCockpit}>
              {item.inCockpit ? "In cockpit" : "Add to cockpit"}
            </Button>
          </ListItem>
        );
      })}
    </StyledList>
  );
}
