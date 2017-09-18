module NewArticle exposing (..)

import Html exposing (..)
import Html.Attributes exposing (..)
import Html.Events exposing (..)
import Http
import Json.Encode

main =
  Html.program
    { init = init 
    , view = view
    , update = update
    , subscriptions = subscriptions
    }

type alias Model =
    { articleTitle: String
    , articleContent: String
    , language: String
    }


init : (Model, Cmd Msg)
init = (Model "" "" "", Cmd.none)

type Msg
    = Init
    | Normal
    | ChangeTitle String
    | ChangeContent String
    | ChangeLanguage String
    | Submit
    | Sending (Result Http.Error String)
    | Error

update : Msg -> Model -> (Model, Cmd Msg)
update msg model =
    case msg of
        Init ->
            (model, Cmd.none)
                
        Normal ->
            (model, Cmd.none)
        
        ChangeTitle newTitle ->
            ({model | articleTitle = newTitle}, Cmd.none)

        ChangeContent newText ->
            ({model | articleContent = newText}, Cmd.none)

        ChangeLanguage newLanguage ->
            ({model | language = newLanguage}, Cmd.none)

        Submit ->
            (model, sendArticleInfo model) 

        Sending (Ok response) ->
            (model, Cmd.none)

        Sending (Err _) ->
            (model, Cmd.none)

        Error ->
            (model, Cmd.none)

view : Model -> Html Msg
view model =
    div []
        [h1 [] [text "Submit an article"]
        , input [placeholder "article title", onInput ChangeTitle] []
        , input [placeholder "article content", onInput ChangeContent] []
        , input [placeholder "article language", onInput ChangeLanguage] []
        , button [onClick Submit] [text "Submit"]
        ]
                
subscriptions : Model -> Sub Msg
subscriptions model =
    Sub.none

sendArticleInfo : Model -> Cmd Msg
sendArticleInfo model =
    Http.send Sending (articleRequest model)


articleRequest : Model -> Http.Request String
articleRequest model = Http.request
                    { method = "POST"
                    , headers = []
                    , url = "/new_article"
                    , body = Http.stringBody "application/json" (Json.Encode.encode 0 (encodeArticleContent model))
                    , expect = Http.expectStringResponse (\v -> Ok v.status.message)
                    , timeout = Nothing
                    , withCredentials = True
                    }

encodeArticleContent : Model -> Json.Encode.Value
encodeArticleContent record =
    Json.Encode.object
        [ ("title", Json.Encode.string record.articleTitle)
        , ("text", Json.Encode.string record.articleContent)
        , ("language_name", Json.Encode.string record.language)
        ]


