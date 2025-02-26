# BioengineerLab

## Описание
BioengineerLab - это система для моделирования и анализа биологических процессов, включая генетические алгоритмы и моделирование клеточных взаимодействий. Система построена на языке Rust с использованием принципов Domain-Driven Design (DDD).

## Структура проекта
Проект разделен на несколько слоев для улучшения читаемости и поддерживаемости кода:

- **Domain**: Основная бизнес-логика и правила.
- **Application**: Интерфейсы, юзкейсы и реализации для работы с данными.
- **Infrastructure**: Реализация деталей инфраструктуры, таких как репозитории данных.
- **Presentation**: Взаимодействие с пользователем.

## Структура каталогов
```plaintext
bioengineer_lab/
├── src/
│   ├── domain/
│   │   ├── entities/
│   │   │   ├── gene.rs
│   │   │   ├── cell.rs
│   │   ├── repositories/
│   │   │   ├── gene_repository.rs
│   │   │   └── cell_repository.rs
│   │   ├── services/
│   │   │   ├── gene_service.rs
│   │   │   └── cell_service.rs
│   │   └── use_cases/
│   │       ├── manage_genes.rs
│   │       └── simulate_cells.rs
├── infrastructure/
│   ├── persistence/
│   │   ├── postgres/
│   │   │   ├── gene_repository_impl.rs
│   │   │   └── cell_repository_impl.rs
│   └── config.rs
├── app/
│   ├── controllers/
│   │   ├── gene_controller.rs
│   │   └── cell_controller.rs
├── main.rs
├── Cargo.toml
├── README.md
└── LICENSE
```

## Установка
1. Клонируйте репозиторий:
    ```bash
    git clone <URL репозитория>
    ```
2. Перейдите в каталог проекта:
    ```bash
    cd bioengineer_lab
    ```
3. Установите необходимые зависимости:
    ```bash
    cargo build
    ```

## Запуск
Для запуска проекта выполните команду:
```bash
cargo run
```

## Описание компонентов

### Domain
- **gene.rs**: Класс сущности гена.
    ```rust
    pub struct Gene {
        pub id: i32,
        pub sequence: String,
    }

    impl Gene {
        pub fn new(id: i32, sequence: String) -> Self {
            Self { id, sequence }
        }
    }
    ```

- **cell.rs**: Класс сущности клетки.
    ```rust
    pub struct Cell {
        pub id: i32,
        pub genes: Vec<Gene>,
    }

    impl Cell {
        pub fn new(id: i32, genes: Vec<Gene>) -> Self {
            Self { id, genes }
        }
    }
    ```

- **gene_repository.rs**: Интерфейс репозитория генов.
    ```rust
    use crate::domain::entities::gene::Gene;

    pub trait GeneRepository {
        fn save_gene(&self, gene: &Gene) -> Result<(), String>;
        fn find_gene_by_id(&self, id: i32) -> Option<Gene>;
        fn find_all_genes(&self) -> Vec<Gene>;
        fn delete_gene(&self, id: i32) -> Result<(), String>;
    }
    ```

- **cell_repository.rs**: Интерфейс репозитория клеток.
    ```rust
    use crate::domain::entities::cell::Cell;

    pub trait CellRepository {
        fn save_cell(&self, cell: &Cell) -> Result<(), String>;
        fn find_cell_by_id(&self, id: i32) -> Option<Cell>;
        fn find_all_cells(&self) -> Vec<Cell>;
        fn delete_cell(&self, id: i32) -> Result<(), String>;
    }
    ```

### Application
- **manage_genes.rs**: Юзкейс для управления генами.
    ```rust
    use crate::domain::entities::gene::Gene;
    use crate::domain::repositories::gene_repository::GeneRepository;

    pub struct ManageGenes<R: GeneRepository> {
        repository: R,
    }

    impl<R: GeneRepository> ManageGenes<R> {
        pub fn new(repository: R) -> Self {
            Self { repository }
        }

        pub fn create_gene(&self, id: i32, sequence: String) -> Result<(), String> {
            let gene = Gene::new(id, sequence);
            self.repository.save_gene(&gene)
        }

        pub fn view_gene(&self, id: i32) -> Option<Gene> {
            self.repository.find_gene_by_id(id)
        }

        pub fn view_all_genes(&self) -> Vec<Gene> {
            self.repository.find_all_genes()
        }

        pub fn remove_gene(&self, id: i32) -> Result<(), String> {
            self.repository.delete_gene(id)
        }
    }
    ```

- **simulate_cells.rs**: Юзкейс для моделирования клеточных взаимодействий.
    ```rust
    use crate::domain::entities::cell::Cell;
    use crate::domain::repositories::cell_repository::CellRepository;

    pub struct SimulateCells<R: CellRepository> {
        repository: R,
    }

    impl<R: CellRepository> SimulateCells<R> {
        pub fn new(repository: R) -> Self {
            Self { repository }
        }

        pub fn simulate(&self) -> Vec<Cell> {
            // Логика симуляции клеточных взаимодействий
            self.repository.find_all_cells()
        }
    }
    ```

### Infrastructure
- **gene_repository_impl.rs**: Реализация репозитория генов с использованием PostgreSQL.
    ```rust
    use crate::domain::entities::gene::Gene;
    use crate::domain::repositories::gene_repository::GeneRepository;
    use postgres::{Client, NoTls};

    pub struct GeneRepositoryImpl {
        client: Client,
    }

    impl GeneRepositoryImpl {
        pub fn new() -> Self {
            let client = Client::connect("host=localhost user=postgres password=secret dbname=bioengineer_db", NoTls).unwrap();
            Self { client }
        }
    }

    impl GeneRepository for GeneRepositoryImpl {
        fn save_gene(&self, gene: &Gene) -> Result<(), String> {
            self.client.execute(
                "INSERT INTO genes (id, sequence) VALUES ($1, $2)",
                &[&gene.id, &gene.sequence],
            ).map_err(|e| e.to_string())?;
            Ok(())
        }

        fn find_gene_by_id(&self, id: i32) -> Option<Gene> {
            let row = self.client.query_opt(
                "SELECT id, sequence FROM genes WHERE id = $1",
                &[&id],
            ).ok()??;
            Some(Gene::new(row.get(0), row.get(1)))
        }

        fn find_all_genes(&self) -> Vec<Gene> {
            let rows = self.client.query("SELECT id, sequence FROM genes", &[]).unwrap();
            rows.into_iter().map(|row| Gene::new(row.get(0), row.get(1))).collect()
        }

        fn delete_gene(&self, id: i32) -> Result<(), String> {
            self.client.execute("DELETE FROM genes WHERE id = $1", &[&id]).map_err(|e| e.to_string())?;
            Ok(())
        }
    }
    ```

- **cell_repository_impl.rs**: Реализация репозитория клеток с использованием PostgreSQL.
    ```rust
    use crate::domain::entities::cell::Cell;
    use crate::domain::repositories::cell_repository::CellRepository;
    use postgres::{Client, NoTls};

    pub struct CellRepositoryImpl {
        client: Client,
    }

    impl CellRepositoryImpl {
        pub fn new() -> Self {
            let client = Client::connect("host=localhost user=postgres password=secret dbname=bioengineer_db", NoTls).unwrap();
            Self { client }
        }
    }

    impl CellRepository for CellRepositoryImpl {
        fn save_cell(&self, cell: &Cell) -> Result<(), String> {
            self.client.execute(
                "INSERT INTO cells (id, genes) VALUES ($1, $2)",
                &[&cell.id, &cell.genes],
            ).map_err(|e| e.to_string())?;
            Ok(())
        }

        fn find_cell_by_id(&self, id: i32) -> Option<Cell> {
            let row = self.client.query_opt(
                "SELECT id, genes FROM cells WHERE id = $1",
                &[&id],
            ).ok()??;
            Some(Cell::new(row.get(0), row.get(1)))
        }

        fn find_all_cells(&self) -> Vec<Cell> {
            let rows = self.client.query("SELECT id, genes FROM cells", &[]).unwrap();
            rows.into_iter().map(|row| Cell::new(row.get(0), row.get(1))).collect()
        }

        fn delete_cell(&self, id: i32) -> Result<(), String> {
            self.client.execute("DELETE FROM cells WHERE id = $1", &[&id]).map_err(|
    ```
    ## Лицензия
   Этот проект лицензирован под лицензией MIT. Для получения дополнительной информации смотрите файл LICENSE.
