// Axumite - A Lightweight API Framework Built on Axum
// Copyright (C) 2025  ProDeSquare
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.
//
// ------------------------------------------------------------
// Axumite was originally built for personal use — structured,
// rate-limited, database-backed, and predictable.
// Anyone is free to use or extend it for personal or commercial work.
// ------------------------------------------------------------

mod app;
mod config;
mod controllers;
mod db;
mod error;
mod extractors;
mod middleware;
mod models;
mod routes;
mod state;

#[tokio::main]
async fn main() {
    app::run().await;
}
