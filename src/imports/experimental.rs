/// Fiber standard variables.
pub const FIBER_STD: &str = "$set_appearance=global/Horse.CloneHorseAppearance;
$set_player_name=global/PlayerName.SetDataString;
$set_horse_name=global/HorseName.SetDataString;
$heal_horse=global/Network.NetworkHorseFreeWellbeing;
$set_player_pos=global/Player.SetPosition;
$set_horse_pos=global/Horse.SetPosition;
$set_horse_mat=global/Horse/Skin/Pelvis.SetSkinMeshSubsetMaterial;
$chat=global/ChatWindow;
$delete_warnings=global/ForcePlayerFromRestrictedArea.Delete;
$move_horse_to=global/Horse.SetOrientationByObject;
$move_player_to=global/Player.SetOrientationByObject;
$spawn_bready=global/QuestManager/Episode1/Chain6004/E01_Quest_L01_C6004_010.QuestComplete;
$set_horse_body=global/Horse.HorseAppearanceCustomBody;
$set_horse_hair=global/Horse.HorseAppearanceCustomHair;
$set_client_sc=global/PlayerStarMoney.SetDataInt;
$set_client_js=global/PlayerMoney.SetDataInt;
$delete_gas=sys.DeleteGlobalAccessShortcut;
$set_gas=GlobalAccessShortcut;
$start_spawner[0]=global/PreShadowWitches.Start;
$stop_spawner[0]=global/PreShadowWitches.Stop;
$flush_spawner[0]=global/PreShadowWitches.FileObjectUnLoad;
$load_spawner[0]=global/PreShadowWitches.FileObjectLoad;
$set_spawner_target[0]=global/PreShadowWitches.SetFileObjectName;
$set_spawner_gas[0]=global/PreShadowWitches.GlobalAccessShortcut;
$start_spawner[1]=global/CableWayExcavator.Start;
$stop_spawner[1]=global/CableWayExcavator.Stop;
$flush_spawner[1]=global/CableWayExcavator.FileObjectUnLoad;
$load_spawner[1]=global/CableWayExcavator.FileObjectLoad;
$set_spawner_target[1]=global/CableWayExcavator.SetFileObjectName;
$set_spawner_gas[1]=global/CableWayExcavator.GlobalAccessShortcut;
$set_horse_speed=global/Horse/Skin/Pelvis.SetSkinMeshAnimationSpeed;
$load_staff_tool=global/GMUI.FileObjectLoad;
$start_staff_tool=global/GMWindow.Start;";

/// Checks for $open_buy_menu(id); and replaces it with the PXScript code.
pub fn check_horse_purchase(code: String) -> String {
    if !code.contains("$open_buy_menu") {
        return code;
    }

    let mut code = code;
    let id = code
        .split('(')
        .nth(1)
        .unwrap_or_else(|| {
            crash!(
                "Invalid $open_buy_menu format! EXAMPLE: $open_buy_menu(10);",
                true
            )
        })
        .replace(");", "");

    code = code.replace(&format!("$open_buy_menu({id});"), &format!(r#"global/HorseForSaleInfoWindow/RenderViews/RenderTargetView/Scene/Animation/Horse.CloneHorse({id}, 1);
global/HorseForSaleInfoWindow/RenderViews/RenderTargetView/Scene/Animation/Horse.HorseForSaleStartBuyWindow();"#));

    code
}
