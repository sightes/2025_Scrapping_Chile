use reqwest::blocking::get;
use scraper::{Html, Selector};

fn main() {
    // URL de la página de valores del dólar 2025
    let url = "https://www.sii.cl/valores_y_fechas/dolar/dolar2024.htm";

    // Realizar la solicitud HTTP
    match get(url) {
        Ok(response) => {
            if response.status().is_success() {
                // Obtener el cuerpo de la respuesta
                let body = response.text().unwrap();

                // Analizar el HTML
                let document = Html::parse_document(&body);

                // Crear un selector para las tablas
                let table_selector = Selector::parse("table").unwrap();
                let row_selector = Selector::parse("tr").unwrap();
                let cell_selector = Selector::parse("td").unwrap();

                // Seleccionar la primera tabla
                if let Some(table) = document.select(&table_selector).next() {
                    // Recorrer las filas de la tabla
                    for row in table.select(&row_selector) {
                        let cells: Vec<_> = row.select(&cell_selector).collect();
                        if cells.len() > 1 {
                            // Mostrar solo el valor del dólar (segunda columna)
                            let valor_dolar = cells[1].text().collect::<Vec<_>>().join("");
                            println!("{}", valor_dolar);
                        }
                    }
                } else {
                    println!("No se encontró ninguna tabla en la página.");
                }
            } else {
                println!("Error al cargar la página: {}", response.status());
            }
        }
        Err(err) => {
            println!("Error al realizar la solicitud: {}", err);
        }
    }
}