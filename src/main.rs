
use std::io;
use thirtyfour::prelude::*;
use thirtyfour::components::SelectElement;
use std::time::Duration;

#[tokio::main]
async fn main() -> WebDriverResult<()>{
    let mut caps = DesiredCapabilities::chrome();
    caps.set_binary("/usr/bin/chromium")?;

    let driver = WebDriver::new("http://localhost:9515", caps).await?;

    driver.maximize_window().await?;


    driver.goto("https://fg.dgii.gov.do/ecf/PortalFG/login?redirectUrl=%2Ffacturador").await?;

    let boton = driver
    .query(By::Css("button.btn.btn-success.mt-2"))
    .and_clickable()
    .first()
    .await?;
    boton.click().await?;

    escrituraInicio(&driver, "TURNC",
     "TU_USUARIO", "TUPASSWORD").await?;
    
    selecionDeTipo(&driver).await?;

    //tipo de pago
    seleccionarTipoPago(&driver, "1", "01", "2","3").await?;

    formularioDos(&driver,"CEDULA","NOMBRE USUARIO","DESCRIPCION DEL PAGO","MONTO").await?;


    println!("Chrome abierto. Presiona Enter para cerrarlo...");

    let mut entrada = String::new();
    io::stdin().read_line(&mut entrada).unwrap();

    driver.quit().await?;
    Ok(())

}

async fn escrituraInicio(driver: &WebDriver, rnc_empresa: &str
    , rnc_delegado: &str, pass_rnc: &str) -> WebDriverResult<()> {
    let campo = driver
        .query(By::Id("txtRncEmpresa"))
        .and_displayed()
        .first()
        .await?;
    campo.clear().await?;
    campo.send_keys(rnc_empresa).await?;
    
    //dos

    let campo_delegado = driver
        .query(By::Id("txtRncDelegado"))
        .and_displayed()
        .first()
        .await?;

    campo_delegado.clear().await?;
    campo_delegado.send_keys(rnc_delegado).await?;
    
    // tres
    let campo_pass = driver
        .query(By::Id("txtPassWord"))
        .and_displayed()
        .first()
        .await?;
    campo_pass.clear().await?;
    campo_pass.send_keys(pass_rnc).await?;

    //bton

    let boton = driver
        .query(By::Id("btnAcceder"))
        .and_clickable()
        .first()
        .await?;
    boton.click().await?;

    Ok(())
}

async fn selecionDeTipo(driver: &WebDriver) -> WebDriverResult<()> {
    for _ in 0..3 {
        let botonTipe = driver
            .query(By::Id("lnkFacturador2"))
            .and_clickable()
            .first()
            .await?;
        botonTipe.click().await?;

        let cambio_vista = driver
            .query(By::Id("ddlTipoPago"))
            .wait(Duration::from_secs(10), Duration::from_millis(250))
            .exists()
            .await?;

        if cambio_vista {
            return Ok(());
        }
    }

    driver.query(By::Id("ddlTipoPago")).first().await?;
    Ok(())
}


//tipo de pago
async fn seleccionarTipoPago(driver: &WebDriver, valorUno: &str, valorDos: &str, valorTres: &str, valorCuatro: &str) -> WebDriverResult<()>{
    //pago
    let elementoPago = driver
        .query(By::Id("ddlTipoPago"))
        .and_displayed()
        .first()
        .await?;
    let selectPago = SelectElement::new(&elementoPago).await?;
    selectPago.select_by_value(valorUno).await?;

    //ingreso
    let elementoTipoIngreso = driver
        .query(By::Id("ddlTipoIngreso"))
        .and_displayed()
        .first()
        .await?;
    let selectIngreso = SelectElement::new(&elementoTipoIngreso).await?;
    selectIngreso.select_by_value(valorDos).await?;

    //servicio
    let selectServicio = driver
        .query(By::Id("ddlIndicadorBoS-1"))
        .and_displayed()
        .first()
        .await?;
    let selectIN = SelectElement::new(&selectServicio).await?;
    selectIN.select_by_value(valorTres).await?;

    //0 itebi
    let selectCeroItebi = driver
        .query(By::Id("ddlIndicadorFactura-1"))
        .and_displayed()
        .first()
        .await?;
    
    let selecCero = SelectElement::new(&selectCeroItebi).await?;
    selecCero.select_by_value(valorCuatro).await?;

    Ok(())
}



async fn formularioDos(driver: &WebDriver, rncOCedula: &str
    , nombreCliente: &str, descripcion: &str, montoPagado: &str) -> WebDriverResult<()> {
        //rnc / cedula cliente
    let campo = driver
        .query(By::Id("txtRncComprador"))
        .and_displayed()
        .first()
        .await?;
    campo.clear().await?;
    campo.send_keys(rncOCedula).await?;


    let autocompletado = driver
        .query(By::Css("#txtRazonSocialComprador[readonly]"))
        .wait(Duration::from_secs(10), Duration::from_millis(250))
        .exists()
        .await?;

    
      println!("autocompletado: {autocompletado}");

    if !autocompletado {
        //dos
        //razon social opcional depende
        for intento in 1..=5 {
            let campo_delegado = driver
                .query(By::Id("txtRazonSocialComprador"))
                .and_displayed()
                .and_enabled()
                .first()
                .await?;

            let _ = campo_delegado.clear().await;
            if let Err(e) = campo_delegado.send_keys(nombreCliente).await {
                println!("intento {intento}: error al escribir: {e}");
            }

            let valor = campo_delegado.prop("value").await?.unwrap_or_default();
            println!("intento {intento}: valor en el campo = {valor:?}");

            if valor == nombreCliente {
                break;
            }

            tokio::time::sleep(Duration::from_secs(1)).await;
        }
    }
    
    // descripcion
    let campo_pass = driver
        .query(By::Id("txtDescripcion-1"))
        .and_displayed()
        .first()
        .await?;
    campo_pass.clear().await?;
    campo_pass.send_keys(descripcion).await?;

    //bton

    // descripcion
    let campo_pass = driver
        .query(By::Id("txtPrecio-1"))
        .and_displayed()
        .first()
        .await?;
    campo_pass.clear().await?;
    campo_pass.send_keys(montoPagado).await?;

    println!("rnc: {:?}", driver.find(By::Id("txtRncComprador")).await?.prop("value").await?);
println!("razon social: {:?}", driver.find(By::Id("txtRazonSocialComprador")).await?.prop("value").await?);

    //generar Boton
    let boton = driver
        .query(By::Id("btnGenerarMostrar"))
        .and_clickable()
        .first()
        .await?;
    boton.click().await?;

    Ok(())
}
