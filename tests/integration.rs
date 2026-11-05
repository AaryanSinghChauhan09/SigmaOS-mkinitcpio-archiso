use sigmaos_mkinitcpio_archiso::*;

#[test]
fn component_is_constructible_and_runnable() {
    // Arrange
    let c = sigmaos_mkinitcpio_archiso::MkinitcpioArchiso::new("sigmaos-mkinitcpio-archiso").expect("valid name");
    // Act
    let out = c.run().expect("run succeeds");
    // Assert
    assert!(out.contains("sigmaos-mkinitcpio-archiso"));
}
