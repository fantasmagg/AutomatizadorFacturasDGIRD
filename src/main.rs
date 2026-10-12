
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

    escrituraInicio(&driver, "TU_RNC",
     "TU_USUARIO/CEDULA", "TU_PASS").await?;
    
    selecionDeTipo(&driver).await?;

    //tipo de pago
    seleccionarTipoPago(&driver, "1", "01", "2","3").await?;

        // fecha
    escribirFecha(&driver, "27/09/2026").await?;

    formularioDos(&driver,"CEDULA/RNC","NOMBRE_DEL_CLIENTE","DESCRIPCION_DEL_PAGO","MONTO: EJEMPLO PONER SOLO 300.0").await?;


    firmarYEnviar(&driver, "RUTA_DEL_CERTIFICADO_identity.p12", "PASS").await?;

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


async fn escribirFecha(driver: &WebDriver, fecha: &str) -> WebDriverResult<()> {
    let campo_fecha = driver
        .query(By::Id("txtFechaEmision"))
        .and_displayed()
        .first()
        .await?;

    // quitar readonly
    driver
        .execute(
            "arguments[0].removeAttribute('readonly');",
            vec![campo_fecha.to_json()?],
        )
        .await?;

    campo_fecha.clear().await?;
    campo_fecha.send_keys(fecha).await?;
    campo_fecha.send_keys(Key::Tab).await?;

    println!("fecha: {:?}", campo_fecha.prop("value").await?);

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
    campo.send_keys(Key::Tab).await?;

    // descripcion
    let campo_pass = driver
        .query(By::Id("txtDescripcion-1"))
        .and_displayed()
        .first()
        .await?;
    campo_pass.clear().await?;
    campo_pass.send_keys(descripcion).await?;

    // precio
    let campo_pass = driver
        .query(By::Id("txtPrecio-1"))
        .and_displayed()
        .first()
        .await?;
    campo_pass.clear().await?;
    campo_pass.send_keys(montoPagado).await?;

    //generar Boton
    let boton = driver
        .query(By::Id("btnGenerarMostrar"))
        .and_clickable()
        .first()
        .await?;
    boton.click().await?;

    // si sale "Este campo es requerido" no se encontro el nombre
    let no_encontrado = driver
        .query(By::XPath("//input[@id='txtRazonSocialComprador']/following-sibling::div[contains(@class,'error')]/p"))
        .wait(Duration::from_secs(5), Duration::from_millis(250))
        .exists()
        .await?;

    println!("no encontrado: {no_encontrado}");

        if no_encontrado {
        //razon social
        for intento in 1..=5 {
            let campo_delegado = driver
                .query(By::Id("txtRazonSocialComprador"))
                .and_displayed()
                .and_enabled()
                .first()
                .await?;

            campo_delegado.click().await?;
            let _ = campo_delegado.clear().await;
            campo_delegado.send_keys(nombreCliente).await?;
            campo_delegado.send_keys(Key::Tab).await?;

            // dar tiempo a angular
            tokio::time::sleep(Duration::from_secs(1)).await;

            let valor = campo_delegado.prop("value").await?.unwrap_or_default();
            println!("intento {intento}: valor en el campo = {valor:?}");

            if valor != nombreCliente {
                continue;
            }

            //generar Boton otra vez
            let boton = driver
                .query(By::Id("btnGenerarMostrar"))
                .and_clickable()
                .first()
                .await?;
            boton.click().await?;

            // si el error ya no sale, se tomo el nombre
            let sigue_error = driver
                .query(By::XPath("//input[@id='txtRazonSocialComprador']/following-sibling::div[contains(@class,'error')]/p"))
                .wait(Duration::from_secs(3), Duration::from_millis(250))
                .exists()
                .await?;

            println!("intento {intento}: sigue error = {sigue_error}");

            if !sigue_error {
                break;
            }
        }
    }

    println!("rnc: {:?}", driver.find(By::Id("txtRncComprador")).await?.prop("value").await?);
    println!("razon social: {:?}", driver.find(By::Id("txtRazonSocialComprador")).await?.prop("value").await?);

    Ok(())
}


async fn firmarYEnviar(driver: &WebDriver, rutaCertificado: &str
    , contrasena: &str) -> WebDriverResult<()> {
    // esperar a que el usuario de click a FIRMAR Y ENVIAR
    println!("Revisa la factura y dale click a FIRMAR Y ENVIAR...");
    let campo_contrasena = driver
        .query(By::XPath("//ngb-modal-window[not(@aria-hidden='true')]//input[@id='txtContrasena']"))
        .wait(Duration::from_secs(600), Duration::from_millis(500))
        .and_displayed()
        .first()
        .await?;

    //cargar certificado (solo en el modal visible)
    let campo_certificado = driver
        .query(By::XPath("//ngb-modal-window[not(@aria-hidden='true')]//input[@id='certificadoFisico']"))
        .first()
        .await?;

    // hacerlo visible para que chromedriver pueda usarlo
    driver
        .execute(
            "arguments[0].style.display='block'; arguments[0].style.opacity='1'; arguments[0].style.visibility='visible';",
            vec![campo_certificado.to_json()?],
        )
        .await?;

    if let Err(e) = campo_certificado.send_keys(rutaCertificado).await {
        println!("error al cargar certificado: {e}");
    }

    // avisar a angular del cambio
    driver
        .execute(
            "arguments[0].dispatchEvent(new Event('change', { bubbles: true }));",
            vec![campo_certificado.to_json()?],
        )
        .await?;

    println!("certificado: {:?}", campo_certificado.prop("value").await?);

    //contrasena
    campo_contrasena.clear().await?;
    campo_contrasena.send_keys(contrasena).await?;

    Ok(())
}