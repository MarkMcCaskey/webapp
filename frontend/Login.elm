module Login exposing (..)

import Html exposing (..)
import Html.Attributes exposing (..)
import Html.Events exposing (..)
import Http
import Navigation
import Json.Decode as Decode


main =
  Html.program
    { init = init 
    , view = view
    , update = update
    , subscriptions = subscriptions
    }

type alias Model =
    { name: String
    }

init : (Model, Cmd Msg)
init = (Model "", Cmd.none)

type Msg
    = Name String
    | Submit 
    | SendName (Result Http.Error String)
    | Done

update : Msg -> Model -> (Model, Cmd Msg)
update msg model =
    case msg of
        Name name ->
            ({ model | name = name }, Cmd.none)

        Submit ->
            (model, sendLoginInfo model.name)

        SendName (Ok val) ->
            ({model | name = val}, Navigation.load "/hello")

        SendName (Err _) ->
            (model, Cmd.none)
        Done ->
            (model, Cmd.none)

view : Model -> Html Msg
view model =
    div []
        [ h1 [] [text "Login"]
        , input [type_ "text", placeholder "Name", onInput Name ] []
        , button [onClick Submit] [text "Login"]
        ]


subscriptions : Model -> Sub Msg
subscriptions model =
    Sub.none


sendLoginInfo : String -> Cmd Msg
sendLoginInfo name =
    Http.send SendName (loginRequest name)


loginRequest : String -> Http.Request String
loginRequest name = Http.request
                    { method = "POST"
                    , headers = []
                    , url = "/login"
                    , body = Http.stringBody "text/plain" name
                    , expect = Http.expectStringResponse (\v -> Ok v.status.message)
                    , timeout = Nothing
                    , withCredentials = True
                    }

