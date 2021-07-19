import React from "react";
import { BrowserRouter as Router, Switch, Route } from "react-router-dom";
import { Zoo } from "./Zoo/Zoo";
import "./App.css";

export function App() {
    return (
        <Router>

            <div className="main-content">
                <Switch>
                    <Route path="/">
                        <Zoo />
                    </Route>
                </Switch>
            </div>
        </Router>
    )

}