use std::result;

use anyhow::Context;
use clap::{Parser, Subcommand};
use client_core::task::{create::CreateTaskRequest, ports::TaskService};
use prettytable::{Table, row};
use uuid::Uuid;

#[derive(Parser)]
#[command(name = "duck-do")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Create { title: String },
    Get { id: Uuid },
    List,
    ListActive,
    Complete { id: Uuid },
}

pub struct CliServer<TS: TaskService> {
    task_service: TS,
}

impl<TS: TaskService> CliServer<TS> {
    pub async fn new(task_service: TS) -> anyhow::Result<Self> {
        Ok(Self { task_service })
    }

    pub async fn run(self) -> anyhow::Result<()> {
        let cli = Cli::parse();
        match cli.command {
            Commands::Create { title } => {
                let req = CreateTaskRequest::new(title);
                self.task_service
                    .create_task(&req)
                    .await
                    .context("Failed to create task")?;
            }
            Commands::Get { id } => {
                let result = self.task_service.get_task(&id).await;
                match result {
                    Ok(task) => {
                        let mut table = Table::new();
                        table.add_row(row!["ID", "Title", "Created", "Completed"]);
                        table.add_row(row![
                            task.id(),
                            task.title(),
                            task.created(),
                            task.completed().map_or("".to_string(), |dt| dt.to_string())
                        ]);
                        table.printstd();
                    }
                    Err(err) => {
                        println!("Error occurred: {}", err);
                    }
                }
            }
            Commands::List => {
                let result = self.task_service.get_tasks().await;
                match result {
                    Ok(tasks) => {
                        let mut table = Table::new();
                        table.add_row(row!["ID", "Title", "Created", "Completed"]);
                        for task in tasks {
                            table.add_row(row![
                                task.id(),
                                task.title(),
                                task.created(),
                                task.completed().map_or("".to_string(), |dt| dt.to_string())
                            ]);
                        }
                        table.printstd();
                    }
                    Err(err) => {
                        println!("Error occurred: {}", err);
                    }
                }
            }
            Commands::ListActive => {
                let result = self.task_service.get_active_tasks().await;
                match result {
                    Ok(tasks) => {
                        let mut table = Table::new();
                        table.add_row(row!["ID", "Title", "Created", "Completed"]);
                        for task in tasks {
                            table.add_row(row![
                                task.id(),
                                task.title(),
                                task.created(),
                                task.completed().map_or("".to_string(), |dt| dt.to_string())
                            ]);
                        }
                        table.printstd();
                    }
                    Err(err) => {
                        println!("Error occurred: {}", err);
                    }
                }
            }
            Commands::Complete { id } => {
                let result = self.task_service.complete_task(&id).await;
                match result {
                    Ok(task_id) => {
                        println!("Task {:?} completed", task_id);
                    }
                    Err(err) => {
                        println!("Error occurred: {}", err);
                    }
                }
            }
        }

        Ok(())
    }
}
