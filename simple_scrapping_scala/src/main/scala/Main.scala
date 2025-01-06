import org.jsoup.Jsoup
import scala.jdk.CollectionConverters._

object ScraperSII2025 {
  def main(args: Array[String]): Unit = {
    // URL de la página para el año 2025
    val url = "https://www.sii.cl/valores_y_fechas/dolar/dolar2024.htm"

    try {
      // Cargar el HTML de la página
      val document = Jsoup.connect(url).get()

      // Seleccionar la tabla donde están los valores del dólar
      val table = document.select("table").first() // Ajusta el selector si es necesario

      // Extraer las filas de la tabla
      val rows = table.select("tr").asScala

      // Recorrer las filas y extraer solo el valor del dólar (segunda columna)
      rows.foreach { row =>
        val cells = row.select("td").asScala
        if (cells.nonEmpty) {
          val valorDolar = cells(1).text() // Segunda columna: valor del dólar
          println(valorDolar)
        }
      }
    } catch {
      case e: Exception => println(s"Error durante el scraping: ${e.getMessage}")
    }
  }
}