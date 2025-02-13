import {
  Box,
  Container,
  List,
  ListItem,
  Paper,
  Tab,
  Table,
  TableBody,
  TableCell,
  TableContainer,
  TableHead,
  TableRow,
  Tabs,
} from "@mui/material";
import MKTourAppBar from "./components/AppBar";
import React, { useEffect, useState } from "react";
import { AxiosResponse } from "axios";
import api from "./api";
import theme from "./theme.ts";
import Bracket from "./components/Bracket.tsx";

interface Team {
  id: number;
  player_ids: number[];
}

/*
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct DragodindeReturn {
    pub id: u64,
    pub name: String,
    pub description: String,
    pub genre: u64,
    pub couleur_finale_id: u64,
    pub parent_pere_id: Option<u64>,
    pub parent_mere_id: Option<u64>,
    pub gestation_nb: Option<u64>,
    pub capacity_ids: Vec::<u64>,
}
*/
interface Dragodinde {
  id: number;
  name: string,
  description: string,
  genre: number,
  couleur: number,
  pere: number,
  mere: number,
  gestation: number,
  capacities: number[];
}

function DragodindesList() {
  const [dragodindes, setDragodinde] = useState<Dragodinde[]>([]);

  useEffect(() => {
    api
      .get("/dragodinde")
      .then((res: AxiosResponse) => {
        setDragodinde(res.data);
      })
      .catch((err) => {
        console.debug(err);
      });
  }, []);

  return (
    <Paper>
      <List>
        {dragodindes.map((dragodinde) => {
          return <ListItem key={dragodinde.id}>{dragodinde.id}</ListItem>;
        })}
      </List>
    </Paper>
  );
}

function TeamList() {
  const [teams, setTeams] = useState<Team[]>([]);

  useEffect(() => {
    api
      .get("/teams")
      .then((res: AxiosResponse) => {
        setTeams(res.data);
      })
      .catch((err) => {
        console.debug(err);
      });
  }, []);

  return (
    <Paper>
      <List>
        {teams.map((team) => {
          return <ListItem key={team.id}>{team.id}</ListItem>;
        })}
      </List>
    </Paper>
  );
}

interface Player {
  team_id: number;
  name: string;
  id: number;
}

function PlayerList({ players, teams }: { players: Player[]; teams: Team[] }) {
  return (
    <TableContainer component={Paper}>
      <Table>
        <TableHead>
          <TableRow sx={{ bgcolor: theme.palette.primary.main }}>
            <TableCell sx={{ color: theme.palette.common.white }}>
              Name
            </TableCell>
            <TableCell align="right" sx={{ color: theme.palette.common.white }}>
              Team
            </TableCell>
            <TableCell align="right" sx={{ color: theme.palette.common.white }}>
              Total score
            </TableCell>
            <TableCell align="right" sx={{ color: theme.palette.common.white }}>
              Ranking
            </TableCell>
          </TableRow>
        </TableHead>
        <TableBody>
          {players.map((player) => (
            <TableRow
              key={player.id}
              sx={{ "&:last-child td, &:last-child th": { border: 0 } }}
            >
              <TableCell>{player.name}</TableCell>
              {/* TODO replace team id by team name */}
              <TableCell align="right">
                {teams.filter((t) => t.id == player.team_id)[0].id}
              </TableCell>
              <TableCell align="right">
                {/* TODO replace id by score */}
                {/* {player.score} */} {player.id}
              </TableCell>
              <TableCell align="right">
                {/* TODO replace id by score */}
                {players.sort((a, b) => a.id - b.id).indexOf(player) + 1}
              </TableCell>
            </TableRow>
          ))}
          <TableRow></TableRow>
        </TableBody>
      </Table>
    </TableContainer>
  );
}

function MKTourTabs() {
  // Current tab index
  const [current, setCurrent] = useState(0);
  // List of players fetched from DB
  const [players, setPlayers] = useState<Player[]>([]);
  // List of teams fetched from DB
  const [teams, setTeams] = useState<Team[]>([]);

  const handleChange = (_: React.SyntheticEvent, newValue: number) => {
      setCurrent(newValue);
  };

  useEffect(() => {
    api
      .get("/players")
      .then((res: AxiosResponse) => {
        setPlayers(res.data);
      })
      .catch((err) => {
        console.debug(err);
      });
  }, []);

  useEffect(() => {
    api
      .get("/teams")
      .then((res: AxiosResponse) => {
        setTeams(res.data);
      })
      .catch((err) => {
        console.debug(err);
      });
  }, []);

  return (
    <Container sx={{ marginTop: "1em" }}>
      <Box width="90%" style={{ margin: "auto" }}>
        <Tabs
          value={current}
          onChange={handleChange}
          centered
          sx={{ marginBottom: "1em" }}
          variant="fullWidth"
        >
          <Tab label="Bracket" />
          <Tab label="Teams" />
          <Tab label="Players" />
        </Tabs>
        {current == 0 && <Bracket />}
        {current == 1 && <DragodindesList />}
        {current == 2 && <PlayerList players={players} teams={teams} />}
      </Box>
    </Container>
  );
}

function App() {
    return (
        <div id="AppDiv" style={{ width: "100%", height: "100%", display: "flex", flexDirection: "column" }}>
            <MKTourAppBar />
            <MKTourTabs />
        </div>
    );
}

export default App;
