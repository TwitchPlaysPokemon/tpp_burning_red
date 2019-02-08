CeruleanPokecenter_Script:
	call Serial_TryEstablishingExternallyClockedConnection
	ld a, CERULEAN_CITY
	ld [wLastBlackoutMap], a
	jp EnableAutoTextBoxDrawing

CeruleanPokecenter_TextPointers:
	dw CeruleanHealNurseText
	dw CeruleanPokecenterText2
	dw CeruleanPokecenterText3
	dw CeruleanTradeNurseText

CeruleanTradeNurseText:
	TX_CABLE_CLUB_RECEPTIONIST

CeruleanHealNurseText:
	TX_POKECENTER_NURSE

CeruleanPokecenterText2:
	TX_FAR _CeruleanPokecenterText2
	db "@"

CeruleanPokecenterText3:
	TX_FAR _CeruleanPokecenterText3
	db "@"
