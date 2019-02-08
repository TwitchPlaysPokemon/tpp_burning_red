SaffronPokecenter_Script:
	call Serial_TryEstablishingExternallyClockedConnection
	ld a, SAFFRON_CITY
	ld [wLastBlackoutMap], a
	jp EnableAutoTextBoxDrawing

SaffronPokecenter_TextPointers:
	dw SaffronHealNurseText
	dw SaffronPokecenterText2
	dw SaffronPokecenterText3
	dw SaffronTradeNurseText

SaffronHealNurseText:
	TX_POKECENTER_NURSE

SaffronPokecenterText2:
	TX_FAR _SaffronPokecenterText2
	db "@"

SaffronPokecenterText3:
	TX_FAR _SaffronPokecenterText3
	db "@"

SaffronTradeNurseText:
	TX_CABLE_CLUB_RECEPTIONIST
